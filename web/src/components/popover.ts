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
      box-shadow: var(--dropspot-box-shadow-inset), var(--dropspot-box-shadow-minor);
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

      &[alignment="top"] {
        position-area: top center;
      }

      &[alignment="center"] {
        position-area: center center;
      }

      &[alignment="bottom"] {
        position-area: bottom center;
      }

      &:popover-open {
        opacity: 1;
      }
    }
  `;

  private popoverRef: Ref<HTMLDivElement> = createRef();

  @property()
  private alignment: "left" | "top" | "center" | "bottom" | "right" = "right";

  connectedCallback(): void {
    super.connectedCallback();
    document.addEventListener("popover-toggle", this.handlePopoverToggle);
  }

  disconnectedCallback(): void {
    super.disconnectedCallback();
    document.removeEventListener("popover-toggle", this.handlePopoverToggle);
  }

  public get isOpen() {
    const { value: popover } = this.popoverRef;

    if (!popover) {
      return false;
    }

    return popover.matches(":popover-open");
  }

  /** Programmatic way to toggle a popover on a certain target element */
  public toggle = (target: HTMLElement): void => {
    const { value: popover } = this.popoverRef;

    if (popover) {
      popover.togglePopover({ source: target });
    }
  };

  /** Programmatic way to close a popover */
  public close = (): void => {
    const { value: popover } = this.popoverRef;

    if (popover) {
      popover.hidePopover();
    }
  };

  private handlePopoverToggle = (e: PopoverToggleEvent) => {
    const { selector, srcElement } = e.detail;

    if (this.matches(selector)) {
      // This event was meant for this popover
      this.toggle(srcElement);
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

export type PopoverToggleEvent = CustomEvent<{
  /** The selector of the popover */
  selector: string;

  /** The element which triggered the event. i.e. a button clicked which should open a popover */
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
