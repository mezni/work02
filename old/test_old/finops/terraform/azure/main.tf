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
  name                     = "momentumfinopsstorageaccount2023"
  resource_group_name      = azurerm_resource_group.finops-rg.name
  location                 = azurerm_resource_group.finops-rg.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
  enable_blob_encryption   = true
  enable_file_encryption   = true
}


resource "azurerm_storage_container" "finops-sc" {
  name                  = "finopscontainer"
  storage_account_name  = azurerm_storage_account.finops-sa.name
  container_access_type = "private"
}

resource "azurerm_data_lake_gen2_filesystem" "finopsadsl" {
  name                 = "finopsadsl"
  storage_account_name = azurerm_storage_account.finops-sa.name
  resource_group_name  = azurerm_resource_group.example.name
}

resource "azurerm_data_lake_gen2_path" "finopsadslpath" {
  name                 = "finopsadslpath"
  file_system_name     = azurerm_data_lake_gen2_filesystem.finopsadslpath.name
  storage_account_name = azurerm_storage_account.finops-sa.name
  resource_group_name  = azurerm_resource_group.finops-rg.name
  path                 = "/momentum"
}
