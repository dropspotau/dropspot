import {
  refreshTokens,
  type Authentication,
  type TokenPair,
  type User,
} from "dropspot-core";
import htmx from "htmx.org";

const LOCALSTORAGE_KEY = "dropspot-auth";
let tokens: TokenPair | null = null;

export const setTokens = async (newTokens: TokenPair): Promise<void> => {
  tokens = newTokens;
  localStorage.setItem(LOCALSTORAGE_KEY, JSON.stringify(newTokens));

  // Set in cookie storage too
  await cookieStore.set("access_token", newTokens.access_token);
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

export const loginAtStartup = async (): Promise<void> => {
  // Load tokens at startup
  const cachedTokens = localStorage.getItem(LOCALSTORAGE_KEY);

  if (!cachedTokens) {
    return;
  }

  const parsedCachedTokens = JSON.parse(cachedTokens);

  const loginResult = await refreshTokens(parsedCachedTokens.refresh_token);
  setTokens(loginResult.tokens);
};

htmx.on("htmx:config:request", (event) => {
  const { detail } = event as HtmxConfigRequestEvent;
  const authToken = getAuth();

  if (authToken) {
    // Add the authentication access token if the user is logged in
    detail.ctx.request.headers["Authorization"] = `Bearer ${authToken.token}`;
  }
});

export type LoginEvent = CustomEvent<{ user: User }>;

declare global {
  interface DocumentEventMap {
    login: LoginEvent;
  }
}
