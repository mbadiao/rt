# Variables
GIT = git
CARGO = cargo

# Cible pour exécuter le projet
run:
	$(CARGO) r


# Cible pour nettoyer les fichiers compilés
clean:
	rm -rf *.ppm

# push
push:
	$(GIT) add .
	$(GIT) commit -m "maj"
	$(GIT) push
