Feature: Recipe CRUD
  Scenario: Create a recipe
    When I create a bread recipe with the description: "A delicious bread recipe"
    Then I receive a 201 status code

  Scenario: Get a recipe
    Given I have a bread recipe
    When I get the recipe
    Then I receive a 200 status code
    And I receive a bread recipe with the description: "A delicious bread recipe"

  Scenario: Update a recipe
    Given I have a bread recipe
    When I update the recipe with the description: "A delicious bread recipe with a new description"
    Then I receive a 200 status code
    And I receive a bread recipe with the description: "A delicious bread recipe with a new description"