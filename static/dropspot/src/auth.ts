import type { Authentication, TokenPair } from "dropspot-core";
import htmx from "htmx.org";

let tokens: TokenPair | null = null;

export const setTokens = (newTokens: TokenPair): void => {
  tokens = newTokens;
};

export const getAuth = (): Authentication | null => {
  if (tokens === null) {
    return null;
  }

  return { token: tokens.access_token };
};

/** Non-exhaustive custom type of the htmx:config:request event. */
type HtmxConfigRequestEvent = CustomEvent<{
  ctx: {
    request: {
      parameters: Record<string, string>;
      headers: Record<string, string>;
    };
  };
}>;

htmx.on("htmx:config:request", (event) => {
  const { detail } = event as HtmxConfigRequestEvent;
  const authToken = getAuth();

  if (authToken) {
    // Add the authentication access token if the user is logged in
    detail.ctx.request.headers["Authorization"] = `Bearer ${authToken.token}`;
  }
});

document.body.addEventListener("htmx:config:request", function (event) {
  console.debug(event);
  console.debug(tokens);
});
