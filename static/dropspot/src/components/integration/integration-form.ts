import {
  upsert_integration_js,
  type GcsIntegrationData,
  type IntegrationData,
  type LocalIntegrationData,
  type StorageType,
} from "dropspot-core";
import { html, css, LitElement, type TemplateResult } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { getAuth } from "../../auth";
import { MdFilledTextField } from "@material/web/textfield/filled-text-field";

const isLocal = (
  data: IntegrationData | null,
): data is LocalIntegrationData => {
  if (data === null) {
    return false;
  }

  return data.hasOwnProperty("folder");
};

const isGcs = (data: IntegrationData | null): data is GcsIntegrationData => {
  if (data === null) {
    return false;
  }

  return data.hasOwnProperty("bucket");
};

@customElement("integration-form")
export class IntegrationFormElement extends LitElement {
  static styles = css``;

  @property()
  private slug: StorageType | null = null;

  @property({ attribute: "is-active", type: Boolean })
  private isActive: boolean = false;

  @state()
  private data: IntegrationData | null = null;

  private handleChange =
    <Type extends IntegrationData>(
      key: keyof Type,
      fallbackData: Type,
      transform: (value: string) => Type[typeof key],
    ): ((e: Event) => void) =>
    (e) => {
      if (!(e.target instanceof MdFilledTextField)) {
        return;
      }

      const value = transform(e.target.value);
      let data = isLocal(this.data) ? this.data : fallbackData;
      this.data = { ...data, [key]: value };
    };

  private renderLocal = (): TemplateResult<1> => {
    const initialData: LocalIntegrationData = { folder: "" };
    const data = isLocal(this.data) ? this.data : initialData;

    return html`
      <md-filled-text-field
        type="text"
        name="folder"
        value="${data.folder}"
        pattern="w+"
        class="settings-field-update-input"
        @change="${this.handleChange("folder", data, (value) => value)}"
      >
      </md-filled-text-field>
    `;
  };

  private renderGcs = (): TemplateResult<1> => {
    const initialData: GcsIntegrationData = { bucket_name: "" };
    let data = isGcs(this.data) ? this.data : initialData;

    return html`
      <md-filled-text-field
        type="text"
        value="${data.bucket_name}"
        pattern="w+"
        class="settings-field-update-input"
        @change="${this.handleChange("bucket_name", data, (value) => value)}"
      >
      </md-filled-text-field>
    `;
  };

  private handleSubmit = async (): Promise<void> => {
    const auth = getAuth();
    console.debug(auth, this.slug, this.isActive, this.data);

    if (!this.slug || !this.data || !auth) {
      return;
    }

    await upsert_integration_js(
      { is_active: this.isActive, data: this.data },
      auth,
      this.slug,
    );
  };

  render() {
    return html`
      FORM ${this.isActive ? "yes" : "no"}
      <input
        type="checkbox"
        name="is_active"
        value="true"
        switch
        .checked=${this.isActive}
      />
      ${this.slug === "local" ? this.renderLocal() : ""}
      ${this.slug === "gcs" ? this.renderGcs() : ""}
      <md-filled-button @click=${this.handleSubmit}>Submit</md-filled-button>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "integration-form": IntegrationFormElement;
  }
}
