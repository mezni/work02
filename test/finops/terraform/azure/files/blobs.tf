terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = ">=3.0.0"
    }
  }
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "finops-rg" {
  name     = "finops-rg"
  location = "Canada East"
}

resource "azurerm_storage_account" "finops-sa" {
  name                     = "finopsstorageaccount2003"
  resource_group_name      = azurerm_resource_group.finops-rg.name
  location                 = azurerm_resource_group.finops-rg.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_storage_container" "finops-sc" {
  name                  = "finopscontainer"
  storage_account_name  = azurerm_storage_account.finops-sa.name
  container_access_type = "private"
}
