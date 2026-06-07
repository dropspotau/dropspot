import {
  upsertIntegration,
  type GcsIntegrationData,
  type IntegrationData,
  type LocalIntegrationData,
  type StorageType,
} from "dropspot-core";
import { MdSwitch } from "@material/web/switch/switch";
import { MdFilledTextField } from "@material/web/textfield/filled-text-field";
import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, property, state } from "lit/decorators.js";

import { getAuth } from "../../auth";
import { applyGlobalStyles } from "../../style";
import { ToastElement } from "../toast";

const getInitialData = (slug: StorageType): IntegrationData => {
  if (slug === "local") {
    return { folder: "" };
  }

  if (slug === "gcs") {
    return { bucket_name: "" };
  }

  if (slug === "s3") {
    return { bucket_name: "" };
  }

  throw new Error(`Cannot get initial data for invalid slug: ${slug}`);
};

/**
 * A form
 */
@customElement("integration-form")
export class IntegrationFormElement extends LitElement {
  static styles = css`
    :host {
      display: flex;
      flex-flow: column;
      gap: 1rem;
    }
  `;

  @property()
  private slug: StorageType | null = null;

  @property({ attribute: "is-active", type: Boolean })
  private isActive: boolean = false;

  @state()
  private data: IntegrationData = {} as IntegrationData; // Gets loaded on mount

  connectedCallback(): void {
    super.connectedCallback();

    if (!this.slug) {
      throw new Error("<integration-form> missing slug");
    }

    this.data = getInitialData(this.slug);

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }

    // Take any existing field attributes and update the initial form
    for (const fieldName in this.dataset) {
      const fieldValue = this.dataset[fieldName];

      if (fieldName in this.data && fieldValue !== undefined) {
        this.data = { ...this.data, [fieldName]: fieldValue };
      }
    }
  }

  /**
   * Updates data of a given integration
   * @param key The field of the integration data to change
   * @param transform Transforms the string value of the form input to the respective data type
   */
  private handleChange =
    <Type extends IntegrationData>(
      key: keyof Type,
      transform: (value: string) => Type[typeof key],
    ): ((e: Event) => void) =>
    (e) => {
      if (!(e.target instanceof MdFilledTextField) || !this.slug) {
        return;
      }

      const value = transform(e.target.value);
      const data = this.data ?? getInitialData(this.slug);
      this.data = { ...data, [key]: value };
    };

  private handleActiveChange = (e: Event): void => {
    const { target } = e;

    if (!(target instanceof MdSwitch)) {
      return;
    }

    this.isActive = target.selected;
  };

  private renderLocal = (): TemplateResult<1> => {
    const data: LocalIntegrationData = { folder: "", ...this.data };

    return html`
      <div class="form-row">
        <span class="form-label">Folder</span>
        <md-filled-text-field
          type="text"
          name="folder"
          value="${data.folder}"
          pattern="w+"
          placeholder="Make sure this folder exists!"
          class="settings-field-update-input"
          @change="${this.handleChange("folder", (value) => value)}"
        >
        </md-filled-text-field>
      </div>
    `;
  };

  private renderGcs = (): TemplateResult<1> => {
    const data: GcsIntegrationData = { bucket_name: "", ...this.data };

    return html`
      <div class="form-row">
        <span class="form-label">Folder</span>
        <md-filled-text-field
          type="text"
          name="bucket"
          label="Bucket"
          value="${data.bucket_name}"
          pattern="w+"
          placeholder="Make sure this bucket exists!"
          class="settings-field-update-input"
          @change="${this.handleChange("bucket_name", (value) => value)}"
        >
        </md-filled-text-field>
      </div>
    `;
  };

  private handleSubmit = async (): Promise<void> => {
    const auth = getAuth();

    if (!this.slug || !this.data || !auth) {
      return;
    }

    await upsertIntegration(
      { is_active: this.isActive, data: this.data },
      auth,
      this.slug,
    );
    ToastElement.create("Updated!", "success");
  };

  render() {
    return html`
      <div class="form">
        <div class="form-row">
          <span class="form-label">Active</span>
          <md-switch
            icons
            .selected=${this.isActive}
            @change=${this.handleActiveChange}
          ></md-switch>
        </div>
        ${this.slug === "local" ? this.renderLocal() : ""}
        ${this.slug === "gcs" ? this.renderGcs() : ""}
        <div class="form-row">
          <span class="form-label">Save</span>
          <div class="form-value-row">
            <md-filled-button class="button-success" @click=${this.handleSubmit}
              >Submit</md-filled-button
            >
          </div>
        </div>
      </div>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "integration-form": IntegrationFormElement;
  }
}
