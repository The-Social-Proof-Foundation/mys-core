```bash
# Check if oracle is healthy
curl http://localhost:8080/health

# Get detailed status
curl http://localhost:8080/status | jq
```

## Development

### Running Tests

```bash
cargo test
```

### Code Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

## Support

For production deployment support or questions:

1. Check the troubleshooting section above
2. Review logs with correlation IDs for specific errors
3. Monitor Prometheus metrics for performance insights
4. Use health check endpoints for operational status

## License

Apache-2.0