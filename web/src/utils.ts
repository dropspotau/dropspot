import type { ApiError } from "dropspot-core";

export const isApiError = (error: any): error is ApiError =>
  typeof error === "object" &&
  error.type === "ApiError" &&
  typeof error.message === "string";
