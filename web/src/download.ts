import {
  download as downloadFile,
  parseLink,
  getFile,
  type Link,
  type File,
} from "dropspot-core";
import { getAuth } from "./auth";
import { ToastElement } from "./components";

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
    try {
      return parseLink(file);
    } catch (e) {
      return null;
    }
  }

  return null;
};

const showFileNotFound = (linkedFileElement: Element): void => {
  linkedFileElement.innerHTML = `
    <dropspot-alert variant="danger">
      <h3 class="text-white no-margin">
          The file you're looking for doesn't exist.
      </h3>
    </dropspot-alert>
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
    file = await getFile(fileId);
  } catch (e) {
    showFileNotFound(linkedFileElement);
    return;
  }

  if (file.is_expired) {
    linkedFileElement.innerHTML = `
      <dropspot-alert variant="danger">
        <h3 class="text-white no-margin">${file.name} has expired</h3>
      </dropspot-alert>
    `;
    return;
  }

  linkedFileElement.innerHTML = `
    <dropspot-alert variant="info">
      <span class="text-white">You've been sent</span>
      <h3 class="text-white no-margin">${file.name}</h3>
      <md-filled-button class="button-white download-button">
        <div class="download-button-contents">
          <span>Preview</span>
          <md-icon>download</md-icon>
          <md-circular-progress indeterminate></md-circular-progress>
        </div>
      </md-filled-button>
    </dropspot-alert>
  `;

  const button = linkedFileElement.querySelector("md-filled-button");

  if (button) {
    button.addEventListener("click", async () => {
      let buffer: Uint8Array<ArrayBuffer>;
      const auth = getAuth();

      if (bufferFileMap.has(file.id)) {
        buffer = bufferFileMap.get(file.id)!;
      } else {
        try {
          button.setAttribute("is-downloading", "");
          buffer = (await downloadFile(
            file.id,
            encryption,
            auth,
          )) as Uint8Array<ArrayBuffer>;
          bufferFileMap.set(file.id, buffer);
          document.dispatchEvent(
            new CustomEvent("file-download", { detail: { file } }),
          );
        } catch (e) {
          console.error(e);
          button.removeAttribute("is-downloading");
          ToastElement.create(
            "Sorry, there was an error viewing the file. Please try again.",
            "danger",
          );
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
      filePreviewElement.setAttribute("mime-type", file.mime_type);
      linkedFileElement.appendChild(filePreviewElement);
    });
  }
};

setTimeout(() => {
  initialiseDownload();
}, 500);
