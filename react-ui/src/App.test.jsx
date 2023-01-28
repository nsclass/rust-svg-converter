import React from "react"
import { render } from "@testing-library/react"
import { App } from "./App"

test("main nav should exist", () => {
  const app = render(<App/>)
  const nav = app.findByTestId("mainNav")
  expect(nav).toBeDefined()
})

