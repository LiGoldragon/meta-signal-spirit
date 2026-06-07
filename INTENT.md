# owner-signal-persona-spirit — intent

This crate is the privileged policy Signal contract for `persona-spirit`.
It is a wire vocabulary crate only: no daemon runtime, actors, sockets,
storage engine, classifier logic, or sema-engine calls live here.

The repository name still says `owner-signal` because the meta-signal
rename has not landed for this component yet. Architecturally this is the
meta policy surface: privileged lifecycle/configuration operations that
ordinary peers must not be able to send over `signal-persona-spirit`.

The public wire uses component-local verbs (`Start`, `Drain`, `Reload`,
`Register`, `Retire`). Sema vocabulary is daemon-side observation
classification only and must not be the public request spine.

The contract depends on the shared Signal frame stack and NOTA codec stack.
When `signal-frame` changes its generated contract requirements, this crate
must migrate with it so production `persona-spirit` can keep depending on
the pushed `main` branch without local patches.
