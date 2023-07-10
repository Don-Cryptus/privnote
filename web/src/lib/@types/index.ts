export type NoteKeys =
  | 'tag'
  | 'note'
  | 'duration_hours'
  | 'manual_password'
  | 'manual_password_confirm'
  | 'destroy_without_confirmation'
  | 'notify_email'
  | 'notify_ref'
  | 'error'
  | 'ok';

export type JSONValue = string | number | boolean | { [x: string]: JSONValue } | Array<JSONValue>;

export type ResponseBody = {
  data?: JSONValue;
  messages?: Messages[];
};

export type Messages = {
  message: string;
  path: NoteKeys;
  key?: string;
};

export type DebugMessages = {
  form?: Messages[];
  captcha?: Messages[];
  data?: Messages[];
}

export type Text = { text: string };
export type Tag = { tag: string };
export type Captcha = Tag & Text;
