import type { Integration } from "dropspot-core";
import { html, css, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { applyGlobalStyles } from "../../style";

@customElement("dropspot-integration")
export class IntegrationElement extends LitElement {
  static styles = css`
    :host {
    }
  `;

  @property()
  private slug: "local" | "gcp" | null = null;

  @state()
  private integration: Integration | null = null;

  connectedCallback() {
    super.connectedCallback();

    if (this.shadowRoot) {
      applyGlobalStyles(this.shadowRoot);
    }

    this.load();
  }

  disconnectedCallback() {
    super.disconnectedCallback();
  }

  private load = async (): Promise<void> => {
    if (!this.slug) {
      return;
    }

    if (this.slug === "local") {
      // get_lo
    }
  };

  render() {
    if (!this.integration) {
      return html`Loading...`;
    }

    return html`
      ${this.integration.slug}
      <div class="file-details-container">
        <div class="integration-title">
          <md-icon class="integration-icon">storage</md-icon>
          <h3 class="file-detail-name no-margin">Local</h3>
        </div>

        {% if let Some(local) = local_integration %}
        <div class="file-details-grid">
          <div class="file-detail-item">
            <span class="file-detail-label">Upload Path</span>
            <div class="file-detail-value-row">
              <md-filled-text-field
                type="text"
                name="bucket_name"
                label="Expire after"
                value="{{ local.upload_path }}"
                min="1"
                max="60"
                pattern="W+"
                hx-patch="/app/settings/integrations/local/update"
                hx-trigger="input delay:750ms"
                hx-target="#local-result"
                hx-swap="innerHTML"
                class="settings-field-update-input"
              >
              </md-filled-text-field>
              <div id="local-result" class="settings-field-update-result"></div>
            </div>
          </div>
        </div>
        {% else %}
        <md-filled-button
          id="create-local"
          hx-post="/app/settings/integrations/local/update"
          hx-vals="{'upload_path': 'files'}"
          >Link File System</md-filled-button
        >
        {% endif %}
      </div>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-integration": IntegrationElement;
  }
}
