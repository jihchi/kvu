# kvu

Simplest tool to update/add key-value pairs/lines

```
┌──────────────────────────┐                                         ┌──────────────────────────┐
│ DB_URI=postgres://db/kvu │         ┌─────────────────────┐         │ DB_URI=postgres://db/kvu │
│ DB_USERNAME=username     │──stdin─▶│ kvu DB_USERNAME=kvu │─stdout─▶│ DB_USERNAME=kvu          │
│ DB_PASSWORD=password     │         └─────────────────────┘         │ DB_PASSWORD=password     │
└──────────────────────────┘                                         └──────────────────────────┘
```

## Usage

### Create new pair

```sh
echo "BUCKET=public" | kvu TOKEN=348a1912
BUCKET=public
TOKEN=348a1912
```

### Update existing pairs

```sh
echo "BUCKET=public" | kvu BUCKET=assets
BUCKET=assets
```
