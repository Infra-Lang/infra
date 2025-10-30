{
  "targets": [
    {
      "target_name": "tree_sitter_infra_binding",
      "include_dirs": [
        "<!(node -p \"require('node-addon-api').include\")",
        "src"
      ],
      "sources": [
        "src/parser.c",
        "src/scanner.c"
      ],
      "cflags_c": [
        "-std=c99",
        "-fPIC",
        "-Isrc/tree_sitter"
      ],
      "defines": [
        "NODE_ADDON_API_ENABLE_NODE_API_FLAGS=1"
      ],
      "conditions": [
        ["OS=='win'", {
          "defines": ["PY_MAJOR_VERSION=3"]
        }]
      ]
    }
  ]
}
