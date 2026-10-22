function main() {

}
function registerPlayingTimer() {
  window.setInterval(() => {
    console.log("Playing Timer")
    let tabs = browser.tabs.query({ audible: true });
    tabs.then(tab => {
      console.log(tab.url);

      if (tab.contains("youtube")) {
        const results = browser.scripting.executeScript({
          target: { tabId: tab.id },
          func: () => document.documentElement.outerHTML
        }).then(() => console.log("hi"));
        getYTMPlaying()
      }
    })
  }, 5000)
}

function getYTMPlaying() {
  console.log("YT");
  const songTitleElement = document.querySelector('yt-formatted-string.ytmusic-player-bar')[0];
  const songChannelElement = document.querySelector('yt-formatted-string & [href^=channel]')[0];

  console.log(`${songTitleElement} - ${songChannelElement}`);

}
document.addEventListener("DOMContentLoaded", registerPlayingTimer);
