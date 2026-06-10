import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, ref, type Ref } from "lit/directives/ref.js";

@customElement("dropspot-popover")
export class PopoverElement extends LitElement {
  static styles = css`
    :host {
      position: relative;
    }

    .popover {
      background-color: #ffffff;
      color: var(--dropspot-primary);
      border-radius: 0.5rem;
      box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.2);
    }

    .popover[popover] {
      /* https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/anchor */
      inset: auto;
      align-self: anchor-center;
      opacity: 0;
      transition:
        opacity 0.2s ease-in-out,
        display 0.2s linear;

      &[alignment="left"] {
        right: calc(anchor(left) + 1rem);
      }

      &[alignment="right"] {
        left: calc(anchor(right) + 1rem);
      }

      &:popover-open {
        opacity: 1;
      }
    }
  `;

  private popoverRef: Ref<HTMLDivElement> = createRef();

  @property()
  private alignment: "left" | "right" = "right";

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
    const popover = this.popoverRef.value;

    if (this.matches(selector) && popover) {
      // This event was meant for this popover
      popover.togglePopover({ source: srcElement });
    }
  };

  render() {
    return html`
      <div
        class="popover"
        alignment="${this.alignment}"
        popover="manual"
        ${ref(this.popoverRef)}
      >
        <slot></slot>
      </div>
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

  interface HTMLElement {
    /** At the time of writing, types don't support `HTMLElement.togglePopover` with a source option. */
    togglePopover(options?: { source?: HTMLElement }): void;
  }

  interface DocumentEventMap {
    "popover-toggle": PopoverToggleEvent;
  }
}
