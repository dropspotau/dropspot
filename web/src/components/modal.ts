import { html, css, LitElement, type PropertyValues } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, ref, type Ref } from "lit/directives/ref.js";

@customElement("dropspot-modal")
export class ModalElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }

    dialog {
      flex-flex: column;
      background-color: #ffffff;
      padding: 1rem;
      border-radius: 0.5rem;
      box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.2);
      max-height: 80%;
      max-width: 80%;
      padding: 2rem;

      &[open] {
        display: flex;
      }
    }
  `;

  @property()
  private open: boolean = false;

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
    return html`<dialog
      ref="${ref(this.dialogRef)}"
      closedby="any"
      @close="${this.handleClose}"
    >
      <slot></slot>
    </dialog>`;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-modal": ModalElement;
  }
}
