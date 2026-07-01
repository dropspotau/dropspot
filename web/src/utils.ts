import type { ApiError } from "dropspot-core";

export const isApiError = (error: any): error is ApiError =>
  typeof error === "object" && typeof error.message === "string";

/**
 * Combines a count and a name of an item into a plural form
 * @param name The name of the item to display
 * @param count The count
 * @returns The pluralised form of the item, or singular if count === 1
 */
export const pluralise = (
  name: string,
  count: number,
): `${number} ${string}${"s" | ""}` => {
  if (count === 1) {
    return `${count} ${name}`;
  }

  return `${count} ${name}s`;
};
