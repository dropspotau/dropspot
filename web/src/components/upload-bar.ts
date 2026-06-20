import {
  upload,
  createLink,
  previewUpload,
  updateFile,
  type UploadResult,
  type Integration,
  type StorageType,
} from "dropspot-core";
import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, state } from "lit/decorators.js";
import { classMap } from "lit/directives/class-map.js";
import type { CloseMenuEvent } from "@material/web/menu/internal/controllers/shared";

import { getAuth } from "../auth";
import { applyGlobalStyles } from "../style";
import { ToastElement } from "./toast";
import { createRef, ref, type Ref } from "lit/directives/ref.js";
import { getExpiresAtOptions, getRemainingTimeText } from "./date-utils";
import { addMinutes, format, parseISO } from "date-fns";
import { createDownloadUrl, saveFileLink } from "../storage";
import { MdSelectOption } from "@material/web/select/select-option";

const FADE_TIMEOUT = 3000;

/**
 * A component which shows upload progress of a file, as well as options about which provider to upload with when multiple are available
 */
@customElement("upload-bar")
export class UploadBarElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
      flex-flow: column;
      padding: 1rem 2rem;
      border-radius: 1rem;
      gap: 1rem;
      background-color: var(--dropspot-dark);
      opacity: 1;
    }

    :host([fading]) {
      /* Fade out over time when disappearing, re-appear immediately when moused over */
      transition: opacity ${FADE_TIMEOUT}ms linear;
      opacity: 0;
    }

    .upload-result-row {
      display: flex;
      flex-flow: row nowrap;
      place-items: center;
      gap: 1rem;
      align-items: center;
    }

    .upload-file-title {
      max-width: 80%;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .integration-select {
      display: flex;
      gap: 0.5rem;
    }

    .integration-button {
      display: flex;
      flex-flow: column;
      place-items: center;
      gap: 0.5rem;
      height: 5rem;
      width: 5rem;
      padding: 1rem;
      background: #ffffff;
      border: 2px solid var(--dropspot-grey-light);
      border-radius: 1rem;
      cursor: pointer;
      transition: background-color 0.2s ease;
      text-decoration: none;
      color: inherit;

      &:hover {
        background-color: var(--dropspot-hover);
      }

      &[disabled] {
        opacity: 0.5;
        cursor: default;
      }
    }

    .integration-button-uploading {
      cursor: default;
      --md-circular-progress-size: 2.5rem;
    }

    .integration-name {
      font-size: 0.95rem;
      font-weight: 600;
      color: #1a1a2e;
    }
  `;

  // NOTE(alec): Can't use a Lit property as this is set calling setAttribute
  private file: File = null!;

  @state()
  private uploadResult: UploadResult | null = null;

  @state()
  private integrations: Integration[] = [];

  @state()
  private isSelectingCustomDate: boolean = false;

  /** Used to prevent the element being removed at the end of the fadeOut timeout if the fade out was cancelled */
  private activeFadeTimeout: number = 0;

  private customExpiresAtInputRef: Ref<HTMLInputElement> = createRef();

  /**
   * The slug of the storage integration method being used to currently upload.
   * @example "local" if uploading to local storage, null if not uploading
   */
  @state()
  private uploadingIntegrationSlug: StorageType | null = null;

  connectedCallback() {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }

    this.verifyUpload().then((canUpload) => {
      const integrations = this.integrations.filter(
        (integration) => integration.is_active,
      );

      if (!canUpload) {
        const integration = integrations.at(0);

        if (!integration) {
          ToastElement.create(
            "No integrations to use for file upload",
            "danger",
          );
          return;
        }

        // From here, the user will select an integration which then triggers startUpload
      }

      const hasOneIntegration = this.integrations.length === 1;

      if (hasOneIntegration) {
        // If only one integration is available, use it
        this.startUpload(this.integrations[0]);
      }
    });

    // Start a fade, unless the mouse is over this element
    this.addEventListener("mouseenter", this.preventFadeOut);
    this.addEventListener("mouseleave", this.fadeOut);
  }

  disconnectedCallback(): void {
    this.removeEventListener("mouseenter", this.preventFadeOut);
    this.removeEventListener("mouseleave", this.fadeOut);
  }

  public static create = (file: File): UploadBarElement => {
    const container = document.querySelector("#recent-uploads");

    if (!container) {
      throw new Error(
        "Cannot create upload bar. No #recent-uploads container.",
      );
    }

    const element = document.createElement("upload-bar");
    element.setFile(file); // Not a plain object, so JSON.stringify results in {}
    container.appendChild(element);

    return element;
  };

  private fadeOut = (): void => {
    // this.setAttribute("fading", "");

    const timeout = setTimeout(() => {
      const isSameTimeout = timeout === this.activeFadeTimeout;

      if (isSameTimeout) {
        // this.remove();
      }
    }, FADE_TIMEOUT);

    this.activeFadeTimeout = timeout;
  };

  private preventFadeOut = (): void => {
    this.activeFadeTimeout = 0;
    this.removeAttribute("fading");
  };

  public setFile = (file: File): void => {
    this.file = file;
  };

  private verifyUpload = async (): Promise<boolean> => {
    const auth = getAuth();
    const uploadPreview = await previewUpload(auth, {
      size: this.file.size,
    });
    const { can_upload: canUpload, integrations } = uploadPreview;
    this.integrations = integrations;

    return canUpload;
  };

  private startUpload = async (integration: Integration): Promise<void> => {
    const fileContents = new Uint8Array(await this.file.arrayBuffer());
    const auth = getAuth();

    let result: UploadResult;
    this.uploadingIntegrationSlug = integration.slug;

    try {
      result = await upload(
        this.file.name,
        fileContents,
        auth,
        integration.slug,
      );
    } catch (e) {
      this.uploadingIntegrationSlug = null;
      ToastElement.create(
        "Sorry, there was an issue uploading the file. Please try again.",
        "danger",
      );
      return;
    }

    this.uploadingIntegrationSlug = null;
    this.uploadResult = result;

    // Save the URL so that the refreshed files list has the URL saved in time
    const link = createLink(result.file.id, result.encryption);
    saveFileLink(result.file.id, link);

    const event: FileUploadEvent = new CustomEvent("file-upload", {
      detail: { upload: result },
      bubbles: true,
    });
    this.dispatchEvent(event);

    // Fade out the bar if the upload completes and the user takes no action
    setTimeout(() => {
      this.fadeOut();
    }, 10000);
  };

  private handleUpdateExpiry = async (
    fileId: string,
    expiresAt: Date,
  ): Promise<void> => {
    const auth = getAuth();

    if (!auth) {
      return;
    }

    try {
      const file = await updateFile(fileId, auth, {
        expires_at: expiresAt.toISOString(),
        max_downloads: undefined,
      });

      if (this.uploadResult) {
        // Reflect any updated fields
        this.uploadResult = { ...this.uploadResult, file };
      }
    } catch (e) {
      ToastElement.create(
        "Sorry, there was an error updating the file. Please try again",
        "danger",
      );
      throw e;
    }
  };

  // @ts-ignore
  private handleUpdateDownloadLimit = async (
    fileId: string,
    maxDownloads: number,
  ): Promise<void> => {
    const auth = getAuth();

    if (!auth) {
      return;
    }

    try {
      const file = await updateFile(fileId, auth, {
        expires_at: undefined,
        max_downloads: maxDownloads,
      });

      if (this.uploadResult) {
        // Reflect any updated fields
        this.uploadResult = { ...this.uploadResult, file };
      }
    } catch (e) {
      ToastElement.create(
        "Sorry, there was an error updating the file. Please try again",
        "danger",
      );
    }
  };

  private renderIntegration = (integration: Integration): TemplateResult<1> => {
    const isUploading = this.uploadingIntegrationSlug === integration.slug;
    const isDisabled = this.uploadingIntegrationSlug !== null && !isUploading;
    const className = classMap({
      "integration-button": true,
      "integration-button-uploading": isUploading,
    });

    const handleClick = (): void => {
      if (isDisabled) {
        // Somehow this fired while another file is already uploading
        return;
      }

      this.startUpload(integration);
    };

    return html`
      <button
        class="${className}"
        @click="${handleClick}"
        .disabled="${isDisabled}"
      >
        ${isUploading
          ? html`<md-circular-progress indeterminate></md-circular-progress>`
          : html`
              <integration-icon slug="${integration.slug}"></integration-icon>
              <span class="integration-name">${integration.name}</span>
            `}
      </button>
    `;
  };

  /** Renders the expires at option with one minute added so setting "1 hour" shows as that rather than immedtiately "59 minutes" */
  private renderExpiryOption = (date: Date): TemplateResult<1> => html`
    <md-select-option value="${date.toISOString()}">
      <div slot="headline">${getRemainingTimeText(addMinutes(date, 1))}</div>
    </md-select-option>
  `;

  private renderCustomDateModal = (
    uploadedFileId: string,
    currentExpiresAt: Date,
  ): TemplateResult<1> => {
    const handleConfirm = (): void => {
      const { value: dateInput } = this.customExpiresAtInputRef;
      const dateTimeString = dateInput?.value;

      if (!dateTimeString) {
        return;
      }

      const dateTime = parseISO(dateTimeString);

      if (dateTime <= new Date()) {
        ToastElement.create("File expiry cannot be in the past", "danger");
        return;
      }

      this.handleUpdateExpiry(uploadedFileId, dateTime).then(() => {
        this.isSelectingCustomDate = false;
      });
    };

    // TOOD(alec): Use the Temporal API when my LSP has it
    const now = new Date();
    const inputDateFormat = "y-MM-dd"; // i.e. "2026-06-16"
    const inputTimeFormat = "HH:mm"; // i.e. "15:45"
    const minDate = `${format(now, inputDateFormat)}T${format(now, inputTimeFormat)}`;
    const value = `${format(currentExpiresAt, inputDateFormat)}T${format(currentExpiresAt, inputTimeFormat)}`;

    return html`
      <dropspot-modal
        .open="${this.isSelectingCustomDate}"
        .preventDefaultClose="${true}"
        @close="${() => {
          this.isSelectingCustomDate = false;
        }}"
      >
        <h3 slot="title" class="no-margin text-primary">
          Select an expiry date
        </h3>
        <input
          type="datetime-local"
          name="expires_at"
          value="${value}"
          min="${minDate}"
          ${ref(this.customExpiresAtInputRef)}
        />
        <md-filled-button slot="footer" @click="${handleConfirm}">
          Confirm
        </md-filled-button>
      </dropspot-modal>
    `;
  };

  /** Renders the bottom row of the upload result when a user is not logged in */
  private renderUnauthedOptions = (
    uploadResult: UploadResult,
    currentExpiresAt: Date,
  ): TemplateResult<1> => html`
    <span>
      File expires in <b>${getRemainingTimeText(currentExpiresAt)}</b> and can
      be downloaded <b>${uploadResult.file.max_downloads} times</b>
    </span>
  `;

  /** Renders the bottom row of the upload result when a user is logged in */
  private renderAuthedOptions = (
    uploadResult: UploadResult,
    currentExpiresAt: Date,
  ): TemplateResult<1> => {
    const handleExpiryChangeCloseMenu = (e: CloseMenuEvent): void => {
      const { initiator } = e.detail;

      if (!(initiator instanceof MdSelectOption)) {
        return;
      }

      const { value } = initiator;

      if (value === "custom") {
        this.isSelectingCustomDate = true;
        return;
      }

      if (this.uploadResult) {
        const parsedValue = parseISO(value);
        this.handleUpdateExpiry(this.uploadResult.file.id, parsedValue);
      }
    };

    const handleDownloadCloseMenu = (e: CloseMenuEvent): void => {
      const { initiator } = e.detail;

      if (!(initiator instanceof MdSelectOption)) {
        return;
      }

      const maxDownloads = parseInt(initiator.value);

      if (this.uploadResult && !isNaN(maxDownloads)) {
        this.handleUpdateDownloadLimit(this.uploadResult.file.id, maxDownloads);
      }
    };

    return html`
      <span>File expires in</span>
      <!-- File expiry -->
      <md-outlined-select @close-menu="${handleExpiryChangeCloseMenu}">
        <md-select-option selected aria-label="custom">
          <div slot="headline">${getRemainingTimeText(currentExpiresAt)}</div>
        </md-select-option>
        ${getExpiresAtOptions().map(this.renderExpiryOption)}
        <md-select-option
          value="custom"
          @click="${() => {
            this.isSelectingCustomDate = true;
          }}"
        >
          <div slot="headline">Custom</div>
        </md-select-option>
      </md-outlined-select>
      <span>and can be downloaded</span>
      <!-- Max downloads -->
      <md-outlined-select @close-menu="${handleDownloadCloseMenu}">
        <md-select-option selected aria-label="Current"
          >${uploadResult.file.max_downloads}</md-select-option
        >
        ${[1, 3, 5, 10].map(
          (option) => html`
            <md-select-option value="${option}">
              <div slot="headline">${option}</div>
            </md-select-option>
          `,
        )}
      </md-outlined-select>
      <span>times</span>
      ${this.renderCustomDateModal(uploadResult.file.id, currentExpiresAt)}
    `;
  };

  render() {
    if (this.uploadResult) {
      // The file has uploaded
      const link = createLink(
        this.uploadResult.file.id,
        this.uploadResult.encryption,
      );
      const url = createDownloadUrl(link);
      const isLoggedIn = !!getAuth();

      // Add one minute so the exact "59 minutes" actually becomes "1 hour"
      const currentExpiresAt = addMinutes(
        parseISO(this.uploadResult.file.expires_at),
        1,
      );

      return html`
        <div class="upload-result-row" style="place-content: space-between;">
          <h3 class="text-white no-margin upload-file-title">
            Uploaded ${this.uploadResult.file.name}
          </h3>
          <copy-button
            value="${url.toString()}"
            class="button-white"
          ></copy-button>
        </div>
        <div class="upload-result-row">
          ${isLoggedIn
            ? this.renderAuthedOptions(this.uploadResult, currentExpiresAt)
            : this.renderUnauthedOptions(this.uploadResult, currentExpiresAt)}
        </div>
      `;
    }

    const isSelectingIntegrations =
      this.integrations.length > 1 && !this.uploadResult;

    if (isSelectingIntegrations) {
      // Multple integrations exist and the user must choose which one to upload to
      return html`
        <h3 class="text-white no-margin">
          Please select where this file should be uploaded to.
        </h3>
        <div class="integration-select">
          ${this.integrations.map(this.renderIntegration)}
        </div>
      `;
    }

    // The file is uploading
    return html`
      <h3 class="text-white no-margin">Uploading ${this.file.name}...</h3>
      <div>
        <!-- Filler for space-between -->
      </div>
    `;
  }
}

export type FileUploadEvent = CustomEvent<{
  upload: UploadResult;
}>;

export type FileDownloadEvent = CustomEvent<{
  file: File;
}>;

declare global {
  interface DocumentEventMap {
    "file-upload": FileUploadEvent;
    "file-download": FileDownloadEvent;
  }
  interface HTMLElementTagNameMap {
    "upload-bar": UploadBarElement;
  }
}
