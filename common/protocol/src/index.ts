import * as v1connect from "../gen/es/macrostack/v1/hello_connect";
import * as v1hello from "../gen/es/macrostack/v1/hello_pb";

export const v1 = { ...v1connect, ...v1hello };
