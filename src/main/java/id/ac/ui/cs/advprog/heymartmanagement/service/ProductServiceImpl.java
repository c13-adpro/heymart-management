package id.ac.ui.cs.advprog.heymartmanagement.service;

import id.ac.ui.cs.advprog.heymartmanagement.model.Product;
import id.ac.ui.cs.advprog.heymartmanagement.repository.ProductRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class ProductServiceImpl implements ProductService{

    @Autowired
    private ProductRepository productRepository;

    @Override
    public Product createProduct(Product product){
        productRepository.createProduct(product);
        return product;
    }

}
