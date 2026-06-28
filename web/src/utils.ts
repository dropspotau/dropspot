import type { ApiError } from "dropspot-core";

export const isApiError = (error: any): error is ApiError =>
  typeof error === "object" && typeof error.message === "string";
