import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, type Ref, ref } from "lit/directives/ref.js";

import type { PopoverElement } from "./popover";

@customElement("dropspot-tooltip")
export class TooltipElement extends LitElement {
  static styles = css`
    :host,
    dropspot-popover {
      background-color: var(--dropspot-dark);
      color: #ffffff;
    }
  `;

  @property({ attribute: "target" })
  private targetSelector: string = "";

  private popoverRef: Ref<PopoverElement> = createRef();

  connectedCallback(): void {
    super.connectedCallback();

    const target = this.getTarget();

    if (!target) {
      throw new Error(
        `Target ${this.targetSelector} not found for tooltip or is not an HTMLElement`,
      );
    }

    target.addEventListener("mouseenter", this.handleSelectorMouseEnter);
    target.addEventListener("mouseleave", this.handleSelectorMouseLeave);
  }

  disconnectedCallback(): void {
    super.disconnectedCallback();

    try {
      const target = this.getTarget();

      // If the mount failed these calls will do nothing
      target.removeEventListener("mouseenter", this.handleSelectorMouseEnter);
      target.removeEventListener("mouseleave", this.handleSelectorMouseLeave);
    } catch (e) {
      // Target doesn't exist so don't bother removing its event listeners
    }
  }

  /**
   * Gets the tooltip's target element, and throws an error if the tooltip's root node doesn't support querySelector
   * or if the target isn't an HTMLElement.
   */
  private getTarget = (): HTMLElement => {
    const rootNode = this.getRootNode();

    if (
      !(rootNode instanceof Document) &&
      !(rootNode instanceof Element) &&
      !(rootNode instanceof ShadowRoot)
    ) {
      throw new Error(
        "Tooltip target root must either be the Document, an Element or a ShadowRoot",
      );
    }

    const target = rootNode.querySelector(this.targetSelector);

    if (!(target instanceof HTMLElement)) {
      throw new Error("Tooltip target was not an HTMLElement");
    }

    return target;
  };

  private handleSelectorMouseEnter = (): void => {
    const { value: popover } = this.popoverRef;
    const target = this.getTarget();

    if (popover) {
      popover.toggle(target);
    }
  };

  private handleSelectorMouseLeave = (): void => {
    const { value: popover } = this.popoverRef;

    if (popover) {
      popover.close();
    }
  };

  render() {
    return html`
      <dropspot-popover
        alignment="top"
        .isDark="${true}"
        ${ref(this.popoverRef)}
      >
        <slot></slot>
      </dropspot-popover>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropspot-tooltip": TooltipElement;
  }
}
