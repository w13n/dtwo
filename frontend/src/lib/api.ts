import { env } from "$env/dynamic/public";
import type { Settings, PaginationInfo, ApiError } from "./types";

const API_URL = env.PUBLIC_API_URL || "http://localhost:3000";

export class ApiClientError extends Error {
  status: number;

  constructor(message: string, status: number) {
    super(message);
    this.name = "ApiClientError";
    this.status = status;
  }
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    let message = `HTTP ${response.status}`;
    try {
      const errorData: ApiError = await response.json();
      message = errorData.error || message;
    } catch {
      // Use default message if JSON parsing fails
    }
    throw new ApiClientError(message, response.status);
  }

  // Handle 204 No Content
  if (response.status === 204) {
    return undefined as T;
  }

  return response.json();
}

export interface GetAllSettingsResult {
  items: Settings[];
  pagination: PaginationInfo;
}

export async function getAllSettings(
  limit?: number,
  offset?: number,
): Promise<GetAllSettingsResult> {
  const params = new URLSearchParams();
  if (limit !== undefined) params.set("limit", limit.toString());
  if (offset !== undefined) params.set("offset", offset.toString());

  const url = `${API_URL}/settings${params.toString() ? `?${params}` : ""}`;
  const response = await fetch(url);

  const items = await handleResponse<Settings[]>(response);
  console.log(response.headers);

  const pagination: PaginationInfo = {
    total: parseInt(response.headers.get("X-Total-Count") || "0", 10),
    limit: parseInt(response.headers.get("X-Limit") || "10", 10),
    offset: parseInt(response.headers.get("X-Offset") || "0", 10),
  };

  console.log(pagination);

  return { items, pagination };
}

export async function getSettingsById(id: string): Promise<Settings> {
  const response = await fetch(`${API_URL}/settings/${id}`);
  return handleResponse<Settings>(response);
}

export async function createSettings(
  data: Record<string, unknown>,
): Promise<Settings> {
  const response = await fetch(`${API_URL}/settings`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  return handleResponse<Settings>(response);
}

export async function updateSettings(
  id: string,
  data: Record<string, unknown>,
): Promise<Settings> {
  const response = await fetch(`${API_URL}/settings/${id}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  return handleResponse<Settings>(response);
}

export async function deleteSettings(id: string): Promise<void> {
  const response = await fetch(`${API_URL}/settings/${id}`, {
    method: "DELETE",
  });
  return handleResponse<void>(response);
}
