import { HandleRequest, HttpRequest, HttpResponse } from "@fermyon/spin-sdk";

const encoder = new TextEncoder();

// Expose an endpoint: /lowercase
// Set a Content-Type: application/json header in the response
// Accept a POST request with a JSON body containing a value property
// Return the payload { message: lowercase_string_of_value }

interface InputPayload {
  value: string;
}

interface OutputPayload {
  message: string;
}

export const handleRequest: HandleRequest = async function (
  request: HttpRequest
): Promise<HttpResponse> {
  const input = request.json() as InputPayload;

  const lowercase = input.value.toLowerCase();

  const output: OutputPayload = {
    message: lowercase,
  };

  return {
    status: 200,
    headers: { "content-type": "application/json" },
    body: encoder.encode(JSON.stringify(output)).buffer,
  };
};
