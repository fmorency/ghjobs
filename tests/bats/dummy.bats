#!/usr/bin/env bats

@test "addition using expr" {
  result="$(expr 2 + 2)"
  [ "$result" -eq 4 ]
}
