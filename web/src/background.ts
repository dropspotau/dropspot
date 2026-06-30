import "./background.css";

/** Sets the background colour to a theme based on the day */
const setBackgroundColour = (): void => {
  const now = new Date();
  const hour = now.getHours();

  let backgroundClass: string | null = null;

  if (hour >= 5 && hour < 10) {
    backgroundClass = "background-morning";
  }

  if (hour >= 10 && hour < 17) {
    backgroundClass = "background-day";
  }

  if (hour >= 17 && hour < 20) {
    backgroundClass = "background-evening";
  }

  if (hour >= 20 && hour < 5) {
    backgroundClass = "background-night";
  }

  if (backgroundClass) {
    document.body.classList.add(backgroundClass);
  }
};

setTimeout(setBackgroundColour, 0);
