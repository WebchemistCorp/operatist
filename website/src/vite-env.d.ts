/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_ASURADA_URL?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
