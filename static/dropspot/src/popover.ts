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
      border-radius: 0.5rem;
    }

    dialog[popover] {
      /* https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/anchor */
      inset: auto;
      left: calc(anchor(right) + 1rem);
      align-self: anchor-center;
      opacity: 1;
      transition: opacity 0.2s ease-in-out;

      &::backdrop {
        display: none;
      }

      &:not([open]) {
        opacity: 0;
      }
    }
  `;

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
    const isOpen =
      dialog instanceof HTMLDialogElement && dialog.hasAttribute("open");

    if (!this.matches(selector)) {
      // Don't react to other popover events
      return;
    }

    if (!isOpen && dialog instanceof HTMLDialogElement) {
      dialog.showPopover({ source: srcElement });
      dialog.setAttribute("open", "");
    } else if (isOpen) {
      setTimeout(() => {
        console.debug("Closing", dialog);
        dialog.close();
      }, 200);
    }
  };

  render() {
    return html`
      <dialog popover ${ref(this.dialogRef)}>
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

  interface HTMLDialogElement {
    /** NOTE(alec): At the time of writing, types don't support `dialog.showPopover` with a source option. */
    showPopover(options?: { source?: HTMLElement }): void;
  }

  interface DocumentEventMap {
    "popover-toggle": PopoverToggleEvent;
  }
}
