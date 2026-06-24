import { html, css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import { createRef, type Ref, ref } from "lit/directives/ref.js";

import type { PopoverElement } from "./popover";

@customElement("dropspot-tooltip")
export class TooltipElement extends LitElement {
  static styles = css`
    :host {
      background-color: var(--dropspot-dark);
    }
  `;

  @property({ attribute: "target" })
  private targetSelector: string = "";

  private popoverRef: Ref<PopoverElement> = createRef();

  connectedCallback(): void {
    super.connectedCallback();

    const target = this.getTarget();

    if (!target) {
      console.error(
        `Target ${this.targetSelector} not found for tooltip or is not an HTMLElement`,
      );
      return;
    }

    target.addEventListener("mouseenter", this.handleSelectorMouseEnter);
    target.addEventListener("mouseleave", this.handleSelectorMouseLeave);
  }

  disconnectedCallback(): void {
    super.disconnectedCallback();

    const target = this.getTarget();

    if (!target) {
      console.error(
        `Target ${this.targetSelector} not found for tooltip or is not an HTMLElement`,
      );
      return;
    }

    target.removeEventListener("mouseenter", this.handleSelectorMouseEnter);
    target.removeEventListener("mouseleave", this.handleSelectorMouseLeave);
  }

  private getTarget = (): HTMLElement | null => {
    const rootNode = this.getRootNode();

    if (!(rootNode instanceof Element) && !(rootNode instanceof ShadowRoot)) {
      console.error(
        "Tooltip target root must either be an Element or ShadowRoot",
      );
      return null;
    }

    const target = rootNode.querySelector(this.targetSelector);

    if (target instanceof HTMLElement) {
      return target;
    }

    return null;
  };

  private handleSelectorMouseEnter = (): void => {
    const { value: popover } = this.popoverRef;
    const target = this.getTarget();

    if (popover && target) {
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
      <dropspot-popover alignment="top" ${ref(this.popoverRef)}>
        <slot></slot>
      </dropspot-popover>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    "dropsopt-tooltip": TooltipElement;
  }
}
