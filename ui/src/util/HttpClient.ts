import { HttpRequestMethod } from "@/enums/HttpRequestMethod";

const request = async (
  url: string,
  authToken: string | null,
  type: HttpRequestMethod,
  body?: unknown | null,
  form?: boolean | null
) => {
  const controller = new AbortController();
  const headers = new Headers();
  if (!form) headers.append("Content-Type", "application/json");
  if (authToken) headers.append("Authorization", `Token ${authToken}`);
  const requestOptions: { [k: string]: any } = {
    method: type,
    headers,
    cache: "no-store",
  };
  const { timeout = 8000 } = requestOptions;
  const id = setTimeout(() => controller.abort(), timeout);
  if (body) requestOptions["body"] = body;
  const response = await fetch(url, {
    ...requestOptions,
    signal: controller.signal,
  });
  if (!response.ok) throw new Error(`There was an error with this request (${response.status})`);
  // TODO: workaround to fix API issue
  const result = await response
    .clone()
    .json()
    .catch(() => response.text());
  clearTimeout(id);
  return result;
};

const getTextError = (e: any) => {
  return e.name === "AbortError"
    ? "The request took too long, please check your Internet connection or try again later."
    : e.message;
};

export default {
  request,
  getTextError,
};
