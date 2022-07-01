start: 
	./scripts/run.sh

deploy-cw20:
	./scripts/deploy-cw20.sh
	./scripts/init-cw20.sh

deploy-escrow:
	./scripts/deploy-escrow.sh
	./scripts/init-escrow.sh

balance:
	./scripts/token-balance.sh

send:
	./scripts/token-send.sh

escrow-create:
	./scripts/escrow-create.sh

escrow-list:
	./scripts/escrow-list.sh

escrow-details:
	./scripts/escrow-details.sh

escrow-approve:
	./scripts/escrow-approve.sh

stop:
	./scripts/stop.sh



