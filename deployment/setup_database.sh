#!/bin/bash

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up the dev_forum database...${NC}"

# MySQL credentials
DB_USER="root"
DB_PASS="212002"
DB_NAME="dev_forum"

# Check if the database already exists
if mysql -u$DB_USER -p$DB_PASS -e "USE $DB_NAME" 2>/dev/null; then
    echo -e "${RED}Database $DB_NAME already exists.${NC}"
    read -p "Do you want to drop and recreate it? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${GREEN}Dropping database $DB_NAME...${NC}"
        mysql -u$DB_USER -p$DB_PASS -e "DROP DATABASE $DB_NAME"
    else
        echo -e "${GREEN}Using existing database.${NC}"
        exit 0
    fi
fi

# Create the database
echo -e "${GREEN}Creating database $DB_NAME...${NC}"
mysql -u$DB_USER -p$DB_PASS -e "CREATE DATABASE $DB_NAME CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci"

echo -e "${GREEN}Database setup completed successfully!${NC}"
echo "You can now run the backend application to initialize the schema." 