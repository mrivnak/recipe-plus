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

When(/I update the recipe with the description: "(.*)"/, async function(desc) {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch("http://" + API_URL + "/recipes/" + this.recipe.id, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ "title": this.recipe.title, "description": desc })
  })
  this.status = response.status
})

When(/I get the ingredients for the recipe/, async function() {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch("http://" + API_URL + "/recipes/" + this.recipe.id + "/ingredients")
  this.status = response.status
  this.ingredients = await response.json()
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

When(/I delete the recipe/, async function() {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch("http://" + API_URL + "/recipes/" + this.recipe.id, {
    method: "DELETE"
  })
  this.status = response.status
})

Then(/I receive a (.*) recipe with the description: "(.*)"/, function(title, desc) {
  assert.strictEqual(this.recipe.title, title)
  assert.strictEqual(this.recipe.description, desc)
})

Then(/I receive a list of ingredients/, function() {
  assert.notStrictEqual(this.ingredients, undefined)
  assert(Array.isArray(this.ingredients))
})

Then(/The list of ingredients is empty/, function() {
  assert.strictEqual(this.ingredients.length, 0)
})