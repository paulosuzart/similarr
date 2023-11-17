# Similarr

The goal is to explore [Shuttle.rs](https://www.shuttle.rs/) by implementing a sort of comparator that would take "
mistaken` data as input. For example `"house"` and `"h2se"` should return true as the 2 here represents
2 missing characters, matching the total len of 5 plus the existing characters match.

An example that would not match is `"house"` and `"h2an"`. They both have len of 5 but characters at
the same index do not match.

## Project URL

The project is available at https://similarr.shuttleapp.rs.
You can then call the `/compare` endpoint providing the strings `a` and `b` for comparison like this:

```bash
curl https://similarr.shuttleapp.rs/compare\?a\=a1a\&b\=a22

{
  "a": "a1a",
  "b": "a22",
  "expanded_a": "a*a",
  "expanded_b": "a**********************",
  "result": false
}
```

## Validation

The input strings can have up to 50 characters (on submit), and the numbers in it can represent up to 30. For
example, `"a55b"` is not allowed. 