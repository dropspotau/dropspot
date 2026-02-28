export const download = (name: string, buffer: Uint8Array<ArrayBuffer>) => {
  const blob = new Blob([buffer], { type: "plain/text" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.click();
};
