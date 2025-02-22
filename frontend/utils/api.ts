import axios from "axios";

const baseOrigin = import.meta.server
  ? `http://${process.env.NUXT_INTERNAL_BACKEND_ADDRESS}`
  : "";

const api = axios.create({
  baseURL: baseOrigin + "/api/v1",
  headers: {
    Accept: "application/json",
  },
});

export default api;
