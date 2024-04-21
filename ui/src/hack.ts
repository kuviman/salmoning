import { r } from "@arrow-js/core";

interface DataSource {
  [index: string]: any;
  [index: number]: any;
}

// This changes the types of reactive to make it more friendly
// for typescript. It kinda lies? but it works nicer xd
export function reactive<T extends DataSource>(data: T): T {
  return r(data) as T;
}
