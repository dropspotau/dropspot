/**
 * Applies global styles to a shadow root of a web component
 * Using global styles in a web component is a bit of an oxymoron, but it's mainly of use to apply utility classes
 *
 * @param shadowRoot The web component shadow root to apply global styles to
 */
export const applyGlobalStyles = (shadowRoot: ShadowRoot): void => {
  // CORS rules apply to external style sheets like ones from Material, so ignore them
  const styleSheets = Array.from(document.styleSheets);
  const internalStyleSheets = styleSheets.filter((sheet) => {
    const isExternal = sheet.href?.startsWith("https");
    return !isExternal;
  });

  const globalSheets = internalStyleSheets.map((x) => {
    const sheet = new CSSStyleSheet();
    const css = Array.from(x.cssRules)
      .map((rule) => rule.cssText)
      .join(" ");
    sheet.replaceSync(css);

    return sheet;
  });

  shadowRoot.adoptedStyleSheets.push(...globalSheets);
};
