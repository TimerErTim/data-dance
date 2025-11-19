import createClient from "openapi-fetch";
import { paths } from "./spec";
import config from "../config";

export const client = createClient<paths>({ baseUrl: config.host + "/api" });