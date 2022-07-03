# kvu

Simplest tool to update/add key-value pairs/lines

```
┌──────────────────────────┐                                         ┌──────────────────────────┐
│ DB_URI=postgres://db/kvu │         ┌─────────────────────┐         │ DB_URI=postgres://db/kvu │
│ DB_USERNAME=username     │──stdin─▶│ kvu DB_USERNAME=kvu │─stdout─▶│ DB_USERNAME=kvu          │
│ DB_PASSWORD=password     │         └─────────────────────┘         │ DB_PASSWORD=password     │
└──────────────────────────┘                                         └──────────────────────────┘
```

