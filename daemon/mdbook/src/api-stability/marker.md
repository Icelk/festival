# Marker
In the documentation for all [`Objects`](../common-objects/common-objects.md), [`JSON-RPC` methods](../json-rpc/json-rpc.md), and [`REST` endpoints](../rest/rest.md) there will be a "marker" defining the stability of that API. It will be 1 of the 4 listed markers here.

## 🟢 Stable
This marks that this API's input/output will never change, and can be relied upon.

## 🟡 Incomplete
This marks that the output of this API may have additions in the future.

The existing inputs/outputs **will not change**, however, additional output may appear.

## 🔴 Unstable
This marks that this API may in the future:
- Have additions to input/output
- Have subtractions to input/output
- Be renamed
- Be completely removed

It should not be relied across different `festivald` versions.

## ⚫️ Deprecated
This marks that this API has been superseded by a better one.

The old API will continue to exist, but it is recommended to use the newer one.

