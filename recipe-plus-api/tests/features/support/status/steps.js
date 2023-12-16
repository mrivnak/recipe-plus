import assert from 'assert';
import { When, Then } from '@cucumber/cucumber';
import fetch from 'node-fetch';

const API_URL = '127.0.0.1:8000';

When("I send a GET request to \\/status", async function () {
  this.status = 523 // Default status code (Origin is unreachable)
  const response = await fetch('http://' + API_URL + '/status');
  this.status = response.status;
});

Then("I receive a {int} status code", function (statusCode) {
  assert.strictEqual(this.status, statusCode);
});