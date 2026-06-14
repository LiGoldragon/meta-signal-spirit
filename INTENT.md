# meta-signal-spirit — intent

This crate is the privileged policy Signal contract for `spirit`.
It is a wire vocabulary crate only: no daemon runtime, actors, sockets,
storage engine, classifier logic, or sema-engine calls live here.

The repository name is `meta-signal-spirit`: this is the meta policy
surface for privileged lifecycle/configuration operations that ordinary
peers must not be able to send over `signal-spirit`.

The public wire uses component-local verbs (`Configure`, `Import`).
`Configure` sets the owner-controlled archive database target; `Import`
restores pre-vetted records with stable identifiers over the owner-only
meta socket. Sema vocabulary is daemon-side observation classification
only and must not be the public request spine.

The contract is schema-derived from `schema/meta-signal.schema` and imports
shared Spirit record nouns from `signal-spirit`. Its default graph is
binary-only rkyv over `signal-frame`; NOTA text is an explicit `nota-text`
feature for examples, CLI-edge projection, and tests.
