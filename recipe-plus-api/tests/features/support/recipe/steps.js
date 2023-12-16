import { Given, Then, When } from "@cucumber/cucumber"
import fetch from "node-fetch"
import assert from "assert"

const API_URL = "127.0.0.1:8000"

When(/I create a (.*) recipe with the description: "(.*)"/, async function(title, desc) {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch("http://" + API_URL + "/recipes", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ "title": title, "description": desc })
  })
  this.status = response.status
})

Given(/I have a (.*) recipe/, async function(title) {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch("http://" + API_URL + "/recipes")
  assert.strictEqual(response.status, 200)

  const recipes = await response.json()
  this.recipe = recipes.find(recipe => recipe.title === title)
  assert.notStrictEqual(this.recipe, undefined)
})

When(/I get the recipe/, async function() {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch("http://" + API_URL + "/recipes/" + this.recipe.id)
  this.status = response.status

  if (response.status === 200) {
    this.recipe = await response.json()
  }
})

Then(/I receive a (.*) recipe with the description: "(.*)"/, function(title, desc) {
  assert.strictEqual(this.recipe.title, title)
  assert.strictEqual(this.recipe.description, desc)
})

