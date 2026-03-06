import { html, css, LitElement } from "lit";
import { customElement } from "lit/decorators.js";
import { createRef, ref, type Ref } from "lit/directives/ref.js";

@customElement("dropspot-popover")
export class PopoverElement extends LitElement {
  static styles = css`
    :host {
      display: contents;
    }

    dialog {
      display: flex;
      flex-flow: column;
      background-color: #ffffff;
      color: var(--dropspot-primary);
    }

    dialog[popover] {
      /* https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/anchor */
      inset: auto;
      left: anchor(right);
      align-self: anchor-center;

      &::backdrop {
        display: none;
      }

      &:not([open]) {
        display: none;
      }
    }
  `;

  private isOpen: boolean = false;
  private dialogRef: Ref<HTMLDialogElement> = createRef();

  connectedCallback() {
    super.connectedCallback();
    document.addEventListener("popover-toggle", this.handlePopoverToggle);
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener("popover-toggle", this.handlePopoverToggle);
  }

  private handlePopoverToggle = (e: PopoverToggleEvent) => {
    const { selector, srcElement } = e.detail;
    const dialog = this.dialogRef.value;

    if (!this.matches(selector)) {
      return;
    }

    if (!this.isOpen && dialog instanceof HTMLDialogElement) {
      // NOTE(alec): At the time of writing, types don't support `dialog.showPopover` with a source option.
      (dialog as any).showPopover({ source: srcElement });
      dialog.setAttribute("open", "");
      this.isOpen = true;
    } else if (this.isOpen && dialog instanceof HTMLDialogElement) {
      dialog.close();
      dialog.removeAttribute("open");
      this.isOpen = false;
    }
  };

  render() {
    return html`
      <dialog popover ${ref(this.dialogRef)} ${this.isOpen ? "open" : ""}>
        <slot></slot>
      </dialog>
    `;
  }
}

type PopoverToggleEvent = CustomEvent<{
  selector: string;
  srcElement: HTMLElement;
}>;

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-popover": PopoverElement;
  }

  interface DocumentEventMap {
    "popover-toggle": PopoverToggleEvent;
  }
}
