import { html, css, LitElement, type PropertyValues } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, ref, type Ref } from "lit/directives/ref.js";

@customElement("dropspot-modal")
export class ModalElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
      --modal-max-height: 80dvh;
      --modal-max-width: 80dvw;
    }

    dialog {
      position: relative;
      flex-flow: column;
      gap: 2rem;
      background-color: #ffffff;
      border-radius: 0.5rem;
      box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.2);
      max-height: var(--modal-max-height);
      max-width: var(--modal-max-width);
      padding: 2rem;

      &[open] {
        display: flex;
      }
    }

    .modal-header {
      display: flex;
      flex-flow: column;
      align-items: center;
      flex: 0 0 2rem;
    }

    .modal-body {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 2rem;
      overflow: auto;
    }

    .modal-footer {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 2rem;
      flex: 0 0 2rem;
    }

    .close-button {
      position: absolute;
      top: 0.5rem;
      right: 0.5rem;
    }
  `;

  /** Whether the modal is open or not */
  @property()
  private open: boolean = false;

  /** Prevents the modal from being closed when the backdrop is clicked */
  @property()
  private preventDefaultClose: boolean = false;

  private dialogRef: Ref<HTMLDialogElement> = createRef();

  protected updated(_changedProperties: PropertyValues): void {
    if (!this.dialogRef.value) {
      return;
    }

    if (this.open) {
      this.dialogRef.value.showModal();
    } else {
      this.dialogRef.value.close();
    }
  }

  private handleClose = (): void => {
    // Dispatch another event up so parent elements know that the modal should be closed
    this.dispatchEvent(new Event("close"));
  };

  render() {
    return html`
      <dialog
        ref="${ref(this.dialogRef)}"
        closedby="${this.preventDefaultClose ? "none" : "any"}"
        @close="${this.handleClose}"
      >
        <div class="modal-header">
          <slot name="title"></slot>
        </div>
        <md-icon-button class="close-button" @click=${this.handleClose}>
          <md-icon>close</md-icon>
        </md-icon-button>
        <div class="modal-body">
          <slot></slot>
        </div>
        <div class="modal-footer">
          <slot name="footer"></slot>
        </div>
      </dialog>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-modal": ModalElement;
  }
}
