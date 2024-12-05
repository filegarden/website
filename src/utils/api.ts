import axios from "axios";

const api = axios.create({
  baseURL: "/api/v1",
  headers: {
    Accept: "application/json",
  },
});

api.interceptors.request.use(undefined, handleError);
api.interceptors.response.use(undefined, handleError);

function handleError(error: AxiosError) {
  const errorBoxes = useErrorBoxes();

  if (error.response) {
    errorBoxes.open({
      message: error.response.status + " " + error.response.statusText,
      code:
        typeof error.response.data === "object"
          ? JSON.stringify(error.response.data)
          : undefined,
    });
  } else {
    errorBoxes.open({
      message: error.message,
    });
  }
}

export default api;
