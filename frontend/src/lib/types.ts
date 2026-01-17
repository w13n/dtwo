export interface Settings {
  id: string;
  [key: string]: unknown;
}

export interface PaginationInfo {
  total: number;
  limit: number;
  offset: number;
}

export interface ApiError {
  error: string;
  status: number;
}

export interface Notification {
  type: "success" | "error";
  message: string;
}
