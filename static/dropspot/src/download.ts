import {
  download_js,
  parse_link_js,
  get_file_js,
  type Link,
  type File,
} from "dropspot-core";

export const download = (name: string, blobUrl: string) => {
  const link = document.createElement("a");
  link.href = blobUrl;
  link.download = name;
  link.click();
};

const tryDetectIdentifier = (): Link | null => {
  const url = new URL(window.location.href);
  const file = url.searchParams.get("file");

  if (file) {
    return parse_link_js(file);
  }

  return null;
};

const showFileNotFound = (linkedFileElement: Element): void => {
  linkedFileElement.innerHTML = `
    <md-icon>error</md-icon>
    <h3 class="text-white no-margin">
        The file you're looking for doesn't exist.
    </h3>
  `;
};

const bufferFileMap: Map<string, Uint8Array<ArrayBuffer>> = new Map();

const initialiseDownload = async (): Promise<void> => {
  const identifier = tryDetectIdentifier();
  const linkedFileElement = document.querySelector("#linked-file");

  if (!identifier || !linkedFileElement) {
    return;
  }

  const { file_id: fileId, encryption } = identifier;
  let file: File;

  try {
    file = await get_file_js(fileId);
  } catch (e) {
    showFileNotFound(linkedFileElement);
    return;
  }

  linkedFileElement.innerHTML = `
      <span class="text-white">You've been sent</span>
      <h3 class="text-white no-margin">${file.name}</h3>
      <md-filled-button class="button-white download-button">
        <div class="download-button-contents">
          <span>Preview</span>
          <md-icon>download</md-icon>
          <md-circular-progress indeterminate></md-circular-progress>
        </div>
      </md-filled-button>
  `;

  const button = linkedFileElement.querySelector("md-filled-button");

  if (button) {
    button.addEventListener("click", async () => {
      let buffer: Uint8Array<ArrayBuffer>;

      if (bufferFileMap.has(file.id)) {
        buffer = bufferFileMap.get(file.id)!;
      } else {
        try {
          button.setAttribute("is-downloading", "");
          buffer = (await download_js(
            file.id,
            encryption,
          )) as Uint8Array<ArrayBuffer>;
          bufferFileMap.set(file.id, buffer);
        } catch (e) {
          console.error(e);
          button.removeAttribute("is-downloading");
          return;
        } finally {
          button.removeAttribute("is-downloading");
        }
      }

      // Delete any existing previews for this file
      const existingPreviews =
        linkedFileElement.querySelectorAll("file-preview");

      for (const preview of existingPreviews) {
        preview.remove();
      }

      const filePreviewElement = document.createElement("file-preview");
      filePreviewElement.setAttribute("name", file.name);
      filePreviewElement.setBuffer(buffer);
      linkedFileElement.appendChild(filePreviewElement);
    });
  }
};

setTimeout(() => {
  initialiseDownload();
}, 500);
