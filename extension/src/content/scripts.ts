import createContentApp from './ContentApp'

console.log('[From the page context] Hello from content_scripts!')
let drpDetected = false;
const BASE_URL = "http://localhost:3000";
/**
 * Extension.js content_script entrypoint. The framework calls this on
 * injection and calls the returned function on HMR/teardown to clean up.
 * Do not invoke it yourself.
 */
async function determineIfDrpRunning() {
  if (drpDetected) {
    return true;
  }
  const item = await fetch(BASE_URL);
  const res = await item.text();

  if (res == "drp.rs") {
    drpDetected = true;
  }
  return drpDetected;

}
interface ExtensionMetadata {
  title: string;
  artist?: string;
  media_type: "music" | "video" | "media";
  subprovider_name: string;
}
async function handleYTM(): Promise<ExtensionMetadata> {
  const titleElement = document.querySelector('ytmusic-player-bar yt-formatted-string.title')
  const posterElement = document.querySelector('ytmusic-player-bar yt-formatted-string.byline')?.children[0]
  console.log(titleElement, posterElement)

  if (!titleElement || !posterElement) throw new Error("Could not get title or artist info.");
  console.log(titleElement.textContent, posterElement.textContent)

  return {
    title: titleElement.textContent,
    artist: posterElement.textContent,
    media_type: "music",
    subprovider_name: "YouTube Music"
  }

}
export default function initial() {
  setInterval(async () => {
    determineIfDrpRunning()
    const isYTMusic = window.location.href.includes('music.youtube');

    if (!isYTMusic) return;
    if (drpDetected) {
      try {
        const details = await handleYTM();
        const item = await fetch(BASE_URL + "/", {
          method: "POST",
          body: JSON.stringify(details)
        });
        const res = await item.json();
        console.log(res);
      } catch(e) {
        console.log("[DRP.RS] error: ", e);
      }


    }

  }, 3000)
  const rootDiv = document.createElement('div')
  rootDiv.setAttribute('data-extension-root', 'true')
  // Isolate the host from page styles (e.g. example.com ships div{opacity:.8},
  // which would otherwise fade the whole widget): the shadow DOM only protects
  // descendants; the host element itself still takes page CSS.
  rootDiv.style.cssText = 'all: initial !important'
  document.body.appendChild(rootDiv)

  // Injecting content_scripts inside a shadow dom
  // prevents conflicts with the host page's styles.
  // This way, styles from the extension won't leak into the host page.
  const shadowRoot = rootDiv.attachShadow({ mode: 'open' })

  const styleElement = document.createElement('style')
  shadowRoot.appendChild(styleElement)


  // Render ContentApp inside shadow root
  const container = createContentApp()
  shadowRoot.appendChild(container)

  // The sidebar asks this page for its title over runtime messaging, and the
  // answer leaves with the UI on teardown so a reload never answers twice.
  chrome.runtime.onMessage.addListener(answerPageTitle)

  return () => {
    chrome.runtime.onMessage.removeListener(answerPageTitle)
    rootDiv.remove()
  }
}

function answerPageTitle(
  message: { type?: string } | undefined,
  _sender: chrome.runtime.MessageSender,
  sendResponse: (answer: { title: string; url: string }) => void
) {
  if (!message || message.type !== 'getPageTitle') return

  sendResponse({ title: document.title, url: location.href })
}

