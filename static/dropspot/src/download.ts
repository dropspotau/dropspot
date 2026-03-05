export const download = (name: string, blobUrl: string) => {
  const link = document.createElement("a");
  link.href = blobUrl;
  link.download = name;
  link.click();
};
