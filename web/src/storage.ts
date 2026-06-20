const LOCALSTORAGE_KEY = "fileLinks";

const getExistingFileLinks = (): Record<string, string> => {
  let items: Record<string, string> = JSON.parse(
    localStorage.getItem(LOCALSTORAGE_KEY) || "{}",
  );

  if (typeof items !== "object") {
    items = {};
  }

  return items;
};

export const getFileLink = (id: string): string | null => {
  const items = getExistingFileLinks();
  return items[id] ?? null;
};

export const saveFileLink = (fileId: string, link: string): void => {
  const items = getExistingFileLinks();
  items[fileId] = link;

  localStorage.setItem(LOCALSTORAGE_KEY, JSON.stringify(items));
};

export const createDownloadUrl = (identifier: string): URL => {
  const url = new URL(window.location.href);
  url.searchParams.set("file", identifier);

  return url;
};
